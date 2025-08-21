/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useSearchStore = defineStore('search', () => {
    const advanceSearch = ref(false)

    const text = ref('')
    const body = ref('')
    const before = ref('')
    const cc = ref('')
    const from = ref('')
    const new_ = ref(false)
    const since = ref('')
    const subject = ref('')
    const to = ref('')
    const unseen = ref(false)
    const seen = ref(false)

    return {
        advanceSearch,
        text,
        body,
        before,
        cc,
        from,
        new_,
        since,
        subject,
        to,
        unseen,
        seen
    }
})