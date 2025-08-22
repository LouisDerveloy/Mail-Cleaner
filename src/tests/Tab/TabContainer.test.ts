/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */


import { mount } from '@vue/test-utils';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { h } from 'vue'; // AJOUT: nécessaire pour le slot de rendu
import TabContainer from '../../Components/Tab/TabContainer.vue';
import TabItem from '../../Components/Tab/TabItem.vue';
import gsap from 'gsap';

describe('TabContainer.vue', () => {
    let spyError: ReturnType<typeof vi.spyOn>;

    beforeEach(() => {
        spyError = vi.spyOn(console, 'error').mockImplementation(() => {});
    });

    afterEach(() => {
        spyError.mockRestore();
    });

    it('appelle gsap.context au montage', () => {
        const wrapper = mount(TabContainer, {
            props: { default_item_selected_index: 0 },
            slots: {
                default: '<div>dummy</div>',
            },
        });

        expect((gsap as any).context).toHaveBeenCalled();
        wrapper.unmount();
    });

    it('ne log pas d’erreurs quand la structure DOM attendue est présente', () => {
        mount(TabContainer, {
            props: { default_item_selected_index: 0 },
            slots: { default: '<div>ok</div>' },
        });

        // Les refs existent dans le template => aucun console.error attendu
        expect(spyError).not.toHaveBeenCalled();
    });

    it('revert le contexte gsap à l’unmount', () => {
        const wrapper = mount(TabContainer, {
            props: { default_item_selected_index: 0 },
            slots: { default: '<div>slot content</div>' },
        });

        const ctx = (globalThis as any).__gsapLastContext;
        expect(ctx).toBeTruthy();

        wrapper.unmount();
        expect(ctx.revert).toHaveBeenCalled();
    });

    it('rend des TabItem dans le slot (test intégration simple)', () => {
        const wrapper = mount(TabContainer, {
            props: { default_item_selected_index: 0 },
            slots: {
                default: {
                    render() {
                        return [
                            h(TabItem, { label: 'Tab A' }),
                            h(TabItem, { label: 'Tab B' }),
                            h(TabItem, { label: 'Tab C' }),
                        ] as any;
                    },
                } as any,
            },
        });

        const spans = wrapper.findAll('span');
        const texts = spans.map((s) => s.text());
        expect(texts).toEqual(expect.arrayContaining(['Tab A', 'Tab B', 'Tab C']));
    });
});
