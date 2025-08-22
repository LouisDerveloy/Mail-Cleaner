/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

import { mount } from '@vue/test-utils';
import { describe, it, expect } from 'vitest';
import TipTool from '@/components/TipTool.vue';

describe('TipTool.vue', () => {
    it('affiche l’icône et le texte fourni', () => {
        const wrapper = mount(TipTool, { props: { text: 'A helpful tip' } });
        expect(wrapper.find('.tiptool-icon').exists()).toBe(true);
        expect(wrapper.find('.tiptool-text').text()).toBe('A helpful tip');
    });
});
