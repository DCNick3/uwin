//
// Created by dcnick3 on 12/4/20.
//

#pragma once

#include "ht/handletable.h"
#include "util/nocopy.h"
#include "util/nomove.h"

namespace uwin::ht {
    template<typename T>
    class handle_holder : util::nocopy {
        handletable* _handletable;
        handle<T> _handle;
    public:
        inline handle_holder() : _handletable(nullptr), _handle(handle<T>::invalid()) {
        }

        inline handle_holder(handletable& handletable, handle<T> handle)
            : _handletable(&handletable), _handle(handle) {
        }

        inline handle_holder(handle_holder&& o) noexcept
            : _handletable(o._handletable), _handle(o._handle) {
            o._handle = handle<T>::invalid();
        }

        inline handle_holder& operator=(handle_holder&& o) noexcept {
            if (&o != this) {
                if (!_handle.is_invalid()) {
                    _handletable->close(_handle);
                }
                _handle = o._handle;
                _handletable = o._handletable;
                o._handle = handle<T>::invalid();
            }
            return *this;
        }

        inline ~handle_holder() {
            if (!_handle.is_invalid()) {
                _handletable->close(_handle);
                _handle = handle<T>::invalid();
            }
        }

        inline std::shared_ptr<T> operator->() const {
            assert(!_handle.is_invalid());
            return _handletable->get(_handle);
        }
    };
}