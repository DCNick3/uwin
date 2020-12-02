//
// Created by dcnick3 on 11/30/20.
//

#pragma once

#include "util/nocopy.h"
#include "ht/kobj.h"

namespace uwin::ht {
    // TODO: implement it fully
    /*
    template<typename T>
    class kobjref : util::nocopy {
        static_assert(std::is_base_of<kobj, T>::value, "T must be a descendant of kobj");

        ctx::process& _process_ctx;
        kobj* _ref;
    public:
        explicit inline kobjref(ctx::process& process_ctx, T* ref) : _process_ctx(process_ctx), _ref(ref) {
            _ref->inccount();
        }

        inline kobjref(kobjref<T> const& o) : _process_ctx(o._process_ctx), _ref(o._ref)
        {}

        inline ~kobjref() noexcept {
            if (_ref != nullptr) {
                _ref->deccount(_process_ctx);
                _ref = nullptr;
            }
        }



        T* operator->() const { return static_cast<T*>(_ref); }

    };*/
}