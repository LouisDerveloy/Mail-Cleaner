/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

import { mount } from '@vue/test-utils';
import { describe, it, expect, vi, beforeEach } from 'vitest';

// Tout ce qui est utilisé dans la factory doit être hoisté:
const h = vi.hoisted(() => ({
    store: {
        advanceSearch: true,
        text: '',
        body: '',
        before: '',
        cc: '',
        from: '',
        since: '',
        subject: '',
        to: '',
        new_: false,
        unseen: false,
        seen: false,
    },
    onSearch: vi.fn(),
}));

vi.mock('../stores/search', () => ({
    useSearchStore: () => h.store,
}));

// Stubs simples pour éviter les effets de bord
vi.mock('../components/Button.vue', () => ({
    default: {
        name: 'Button',
        template: '<button @click="$emit(\'click\')"><slot /></button>',
    },
}));
vi.mock('./Input.vue', () => ({
    default: {
        name: 'Input',
        props: ['modelValue', 'label', 'type', 'isDate'],
        emits: ['update:modelValue'],
        template: `<label><slot />{{label}}<input :type="type ?? 'text'" @input="$emit('update:modelValue', $event.target.value)"/></label>`,
    },
}));

import AdvanceSearch from '../Components/advanceSearch.vue';

describe('advanceSearch.vue', () => {
    beforeEach(() => {
        Object.assign(h.store, { advanceSearch: true });
        h.onSearch.mockClear();
    });

    it('affiche le modal quand advanceSearch=true', () => {
        const wrapper = mount(AdvanceSearch, { props: { onSearch: h.onSearch } });
        expect(wrapper.find('.modal-overlay').exists()).toBe(true);
    });

    it('ferme le modal au clic sur Close', async () => {
        const wrapper = mount(AdvanceSearch, { props: { onSearch: h.onSearch } });
        await wrapper.get('.close-btn').trigger('click');
        expect(h.store.advanceSearch).toBe(false);
    });

    it('soumet le formulaire et appelle onSearch', async () => {
        const wrapper = mount(AdvanceSearch, { props: { onSearch: h.onSearch } });
        await wrapper.get('form').trigger('submit.prevent');
        expect(h.store.advanceSearch).toBe(false);
        expect(h.onSearch).toHaveBeenCalled();
    });
});
