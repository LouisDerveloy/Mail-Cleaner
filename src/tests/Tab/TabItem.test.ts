/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LossuisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */

// src/tests/TabItem.test.ts
import { mount } from '@vue/test-utils';
import { describe, it, expect, vi } from 'vitest';
import TabItem from '../../Components/Tab/TabItem.vue';

describe('TabItem.vue', () => {
    it('affiche label et image quand fournis', () => {
        const wrapper = mount(TabItem, {
            props: {
                label: 'Inbox',
                ico: 'icon.svg',
                alt: 'inbox icon',
            },
        });

        const btn = wrapper.get('button.TabItem');
        expect(btn.exists()).toBe(true);

        const img = wrapper.get('img');
        expect(img.attributes('src')).toBe('icon.svg');
        expect(img.attributes('alt')).toBe('inbox icon');

        const text = wrapper.get('span');
        expect(text.text()).toBe('Inbox');
    });

    it('n’affiche pas d’image si ico vide', () => {
        const wrapper = mount(TabItem, {
            props: {
                label: 'Only text',
                ico: '',
            },
        });
        expect(wrapper.find('img').exists()).toBe(false);
        expect(wrapper.find('span').text()).toBe('Only text');
    });

    it('n’affiche pas de span si label vide', () => {
        const wrapper = mount(TabItem, {
            props: {
                label: '',
                ico: 'icon.svg',
            },
        });
        expect(wrapper.find('span').exists()).toBe(false);
        expect(wrapper.find('img').exists()).toBe(true);
    });

    it('déclenche props.onClick au clic', async () => {
        const onClick = vi.fn();
        const wrapper = mount(TabItem, {
            props: { label: 'Clickable', onClick },
        });

        await wrapper.get('button').trigger('click');
        expect(onClick).toHaveBeenCalledTimes(1);
    });
});
