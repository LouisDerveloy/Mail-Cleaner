/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

import { mount } from '@vue/test-utils';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import Button from '@/components/Button.vue';
import gsap from 'gsap';

describe('Button.vue', () => {
    beforeEach(() => vi.clearAllMocks());

    it('applique les classes variant/size/expand et rend le slot', () => {
        const wrapper = mount(Button, {
            props: { onClick: vi.fn(), variant: 'secondary', size: 'large', expand: true },
            slots: { default: 'Action' },
        });
        const btn = wrapper.get('button');
        expect(btn.text()).toContain('Action');
        expect(btn.classes()).toEqual(expect.arrayContaining(['secondary', 'large', 'expand']));
    });

    it('émet onClick quand non disabled', async () => {
        const onClick = vi.fn();
        const wrapper = mount(Button, { props: { onClick }, slots: { default: 'Run' } });
        await wrapper.get('button').trigger('click');
        expect(onClick).toHaveBeenCalledTimes(1);
    });

    it('n’émet pas onClick quand disabled', async () => {
        const onClick = vi.fn();
        const wrapper = mount(Button, { props: { onClick, disabled: true }, slots: { default: 'Run' } });
        await wrapper.get('button').trigger('click');
        expect(onClick).toHaveBeenCalledTimes(0);
    });

    it('attache des animations mousedown/mouseup via gsap', async () => {
        const wrapper = mount(Button, { props: { onClick: vi.fn() }, slots: { default: 'Anim' } });
        const btn = wrapper.get('button');
        await btn.trigger('mousedown');
        await btn.trigger('mouseup');
        // @ts-ignore - mock dans setup
        expect(gsap.to).toHaveBeenCalled();
    });
});
