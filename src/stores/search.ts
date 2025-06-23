import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useSearchStore = defineStore('search', () => {
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