/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

import { mount } from '@vue/test-utils';
import { describe, it, expect } from 'vitest';
import SearchBar from '@/components/SearchBar.vue';

// Stub Button: renvoie un bouton natif qui émet un click
const ButtonStub = {
    template: '<button class="btn-stub" @click="$emit(\'click\')"><slot /></button>',
};

describe('SearchBar.vue', () => {
    it('met à jour v-model et émet "search" sur Enter', async () => {
        const wrapper = mount(SearchBar, {
            global: { stubs: { Button: ButtonStub } },
            props: { modelValue: '' },
        });
        const input = wrapper.get('input[type="text"]');
        await input.setValue('hello');
        await input.trigger('keyup.enter');
        expect(wrapper.emitted('search')).toBeTruthy();
    });

    it('émet "search" au clic sur le bouton', async () => {
        const wrapper = mount(SearchBar, {
            global: { stubs: { Button: ButtonStub } },
            props: { modelValue: 'abc' },
        });
        await wrapper.get('.btn-stub').trigger('click');
        expect(wrapper.emitted('search')).toBeTruthy();
    });

    it('désactive input quand disabled=true', () => {
        const wrapper = mount(SearchBar, {
            global: { stubs: { Button: ButtonStub } },
            props: { modelValue: '', disabled: true },
        });
        expect(wrapper.get('input').attributes('disabled')).toBeDefined();
    });
});
