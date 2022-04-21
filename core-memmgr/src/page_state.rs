use bitflags::bitflags;
use memory_image::Protection;

// A public API
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PageState {
    Uncommitted,
    Committed(Protection),
}

// the value actually stored
bitflags! {
    pub(crate) struct PageStateRepr: u8 {
        const READ = (1 << 1);
        const WRITE = (1 << 2);
        const EXECUTE = (1 << 3);
        const COMMIT = (1 << 4);
    }
}

impl PageState {
    pub fn committed(&self) -> bool {
        match self {
            PageState::Uncommitted => false,
            PageState::Committed(_) => true,
        }
    }
}

impl From<PageStateRepr> for PageState {
    #[inline]
    fn from(repr: PageStateRepr) -> Self {
        let mut protection = Protection::empty();

        // hopefully the compiler ¿can? optimize this down to assignment
        if repr.contains(PageStateRepr::READ) {
            protection |= Protection::READ;
        }
        if repr.contains(PageStateRepr::WRITE) {
            protection |= Protection::WRITE;
        }
        if repr.contains(PageStateRepr::EXECUTE) {
            protection |= Protection::EXECUTE;
        }

        let committed = repr.contains(PageStateRepr::COMMIT);

        if committed {
            Self::Committed(protection)
        } else {
            Self::Uncommitted
        }
    }
}

impl From<PageState> for PageStateRepr {
    #[inline]
    fn from(state: PageState) -> Self {
        let mut res = PageStateRepr::empty();

        match state {
            PageState::Uncommitted => {}
            PageState::Committed(prot) => {
                res |= PageStateRepr::COMMIT;

                if prot.contains(Protection::READ) {
                    res |= PageStateRepr::READ;
                }
                if prot.contains(Protection::WRITE) {
                    res |= PageStateRepr::WRITE;
                }
                if prot.contains(Protection::EXECUTE) {
                    res |= PageStateRepr::EXECUTE;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::{PageState, PageStateRepr, Protection};

    fn is_round_trip_state(state: PageState) -> bool {
        let repr: PageStateRepr = state.into();
        let state_from_repr: PageState = repr.into();
        state_from_repr == state
    }
    fn is_round_trip_repr(repr: PageStateRepr) -> bool {
        let state: PageState = repr.into();
        let repr_from_state: PageStateRepr = state.into();
        repr_from_state == repr
    }

    #[test]
    fn round_trip_state() {
        assert!(is_round_trip_state(PageState::Uncommitted));

        assert!(is_round_trip_state(PageState::Committed(Protection::NONE)));
        assert!(is_round_trip_state(PageState::Committed(Protection::READ)));
        assert!(is_round_trip_state(PageState::Committed(Protection::WRITE)));
        assert!(is_round_trip_state(PageState::Committed(
            Protection::EXECUTE
        )));
        assert!(is_round_trip_state(PageState::Committed(
            Protection::READ_WRITE
        )));
        assert!(is_round_trip_state(PageState::Committed(
            Protection::READ_EXECUTE
        )));
        assert!(is_round_trip_state(PageState::Committed(
            Protection::WRITE_EXECUTE
        )));
    }

    #[test]
    fn round_trip_repr() {
        assert!(is_round_trip_repr(PageStateRepr::empty()));

        assert!(is_round_trip_repr(PageStateRepr::COMMIT));
        assert!(is_round_trip_repr(
            PageStateRepr::COMMIT | PageStateRepr::READ
        ));
        assert!(is_round_trip_repr(
            PageStateRepr::COMMIT | PageStateRepr::WRITE
        ));
        assert!(is_round_trip_repr(
            PageStateRepr::COMMIT | PageStateRepr::EXECUTE
        ));
        assert!(is_round_trip_repr(
            PageStateRepr::COMMIT | PageStateRepr::READ | PageStateRepr::WRITE
        ));
        assert!(is_round_trip_repr(
            PageStateRepr::COMMIT | PageStateRepr::READ | PageStateRepr::EXECUTE
        ));
        assert!(is_round_trip_repr(
            PageStateRepr::COMMIT | PageStateRepr::WRITE | PageStateRepr::EXECUTE
        ));
        assert!(is_round_trip_repr(
            PageStateRepr::COMMIT
                | PageStateRepr::READ
                | PageStateRepr::WRITE
                | PageStateRepr::EXECUTE
        ));
    }
}
