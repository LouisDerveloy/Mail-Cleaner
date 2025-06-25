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