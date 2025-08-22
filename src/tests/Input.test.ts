/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

import { mount } from '@vue/test-utils';
import { describe, it, expect } from 'vitest';
import Input from '@/components/Input.vue';

describe('Input.vue', () => {
    it('text: v-model et clear', async () => {
        const wrapper = mount(Input, { props: { modelValue: 'abc', label: 'Name', type: 'text' } });
        const el = wrapper.get('input');
        expect((el.element as HTMLInputElement).value).toBe('abc');
        await el.setValue('xyz');
        expect(wrapper.emitted('update:modelValue')?.[0]?.[0]).toBe('xyz');

        await wrapper.setProps({ modelValue: 'xyz' });
        await wrapper.get('img.clear-btn').trigger('click');
        expect(wrapper.emitted('update:modelValue')?.at(-1)?.[0]).toBe('');
    });

    it('password: v-model', async () => {
        const wrapper = mount(Input, { props: { modelValue: '', label: 'Pwd', type: 'password' } });
        const el = wrapper.get('input[type="password"]');
        await el.setValue('secret');
        expect(wrapper.emitted('update:modelValue')?.[0]?.[0]).toBe('secret');
    });

    it('checkbox: v-model booléen', async () => {
        const wrapper = mount(Input, { props: { modelValue: false, label: 'OK', type: 'checkbox' } });
        const el = wrapper.get('input[type="checkbox"]');
        await el.setValue(true);
        expect(wrapper.emitted('update:modelValue')?.[0]?.[0]).toBe(true);
    });

    it('number: v-model, respecte min/max côté attributs', () => {
        const wrapper = mount(Input, { props: { modelValue: 2, label: 'Qty', type: 'number', min: 1, max: 10 } });
        const el = wrapper.get('input[type="number"]');
        expect(el.attributes('min')).toBe('1');
        expect(el.attributes('max')).toBe('10');
    });

    it('date: dd-Mon-yyyy -> yyyy-mm-dd (affichage)', () => {
        const wrapper = mount(Input, { props: { modelValue: '1-Jan-2025', label: 'Since', type: 'date' } });
        const el = wrapper.get('input[type="date"]');
        expect((el.element as HTMLInputElement).value).toBe('2025-01-01');
    });

    it('date: yyyy-mm-dd -> dd-Mon-yyyy (emit)', async () => {
        const wrapper = mount(Input, { props: { modelValue: '', label: 'Since', type: 'date' } });
        const el = wrapper.get('input[type="date"]');
        await el.setValue('2025-08-22');
        const last = wrapper.emitted('update:modelValue')?.at(-1)?.[0];
        expect(last).toBe('22-Aug-2025');
    });
});
