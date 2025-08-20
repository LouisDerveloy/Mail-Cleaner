<script setup lang="ts">
import gsap from "gsap";
import {onMounted, onBeforeUnmount, ref} from "vue";

const props = withDefaults(defineProps<{ default_item_selected_index?: number }>(), {default_item_selected_index: 0})

const container = ref<HTMLElement | null>(null);
const hover_bg = ref<HTMLElement | null>(null);
const selected_bg = ref<HTMLElement | null>(null);
const default_item_selected_index = props.default_item_selected_index + 2; // +2 to avoid .hover_bg and .selected_bg
var ctx: gsap.Context | null = null;

onMounted(() => {
  ctx = gsap.context(() => {

    if (container.value === null || hover_bg.value === null || selected_bg.value === null) {
      console.error('Container, selected_bg or hover_bg is null');
      console.error(`Container : ${container.value}`);
      console.error(`Hover_bg : ${hover_bg.value}`);
      console.error(`Selected_bg : ${selected_bg.value}`);
      return;
    }

    const items = container.value.children;
    if (items.length >= default_item_selected_index) move_selected_bg_to(items[default_item_selected_index] as HTMLElement);


    container.value?.addEventListener('mouseenter', () => {
      gsap.to(hover_bg.value, {
        opacity: 1,
        duration: .3,
        ease: 'power2.out',
      });
    });

    container.value?.addEventListener('mouseleave', () => {
      gsap.to(hover_bg.value, {
        opacity: 0,
        duration: .3,
        ease: 'power2.out',
      });
    });

    for (let i = 0; i < items.length; i++) {
      (items[i] as HTMLElement).addEventListener('mouseenter', () => move_hover_bg_to(items[i] as HTMLElement));
      (items[i] as HTMLElement).addEventListener('mousedown', () => move_selected_bg_to(items[i] as HTMLElement));
    }

  }, container.value!);
});
onBeforeUnmount(() => ctx?.revert())

function move_hover_bg_to(target: HTMLElement) {
  if (hover_bg.value === null) {
    console.error("hover_bg is null", target);
    return;
  }

  const hoverRect = hover_bg.value.getBoundingClientRect();
  const targetRect = target.getBoundingClientRect();

  const _x = targetRect.x - hoverRect.x;
  const _y = targetRect.y - hoverRect.y;

  gsap.to(hover_bg.value, {
    x: `+=${_x}`,
    y: `+=${_y}`,
    width: targetRect.width,
    height: targetRect.height,
    duration: .5,
    ease: "power2.out",
  });
}

function move_selected_bg_to(target: HTMLElement) {
  if (selected_bg.value === null) {
    console.error("hover_bg is null", target);
    return;
  }

  const hoverRect = selected_bg.value.getBoundingClientRect();
  const targetRect = target.getBoundingClientRect();

  const _x = targetRect.x - hoverRect.x;
  const _y = targetRect.y - hoverRect.y;

  gsap.to(selected_bg.value, {
    x: `+=${_x}`,
    y: `+=${_y}`,
    width: targetRect.width,
    height: targetRect.height,
    duration: 1,
    ease: "elastic.inOut",
  })  ;
}

</script>

<template>
  <section ref="container" class="TabContainer">
    <div ref="hover_bg" class="hover-bg"></div>
    <div ref="selected_bg" class="selected-bg"></div>
    <slot/>
  </section>
</template>

<style scoped>
.TabContainer {
  position: relative;
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  justify-content: start;
  align-items: center;
  gap: var(--xs-spacing);
}

.hover-bg, .selected-bg {
  top: 0;
  left: 0;
  position: absolute;
  opacity: 0;
  cursor: none;
  pointer-events: none;
  border-radius: var(--l-border-radius);
}

.hover-bg {
  background-color: var(--secondary-color);
  z-index: 1;
}

.selected-bg {
  top: -50px;
  opacity: 1;
  background-color: var(--primary-color);
  z-index: 2;
}
</style>