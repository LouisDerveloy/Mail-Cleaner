<script setup lang="ts">
import gsap from 'gsap';
import {Observer} from 'gsap/Observer';
import {onMounted, onBeforeUnmount, ref} from "vue";

gsap.registerPlugin(Observer)

const props = withDefaults(
    defineProps<{
      variant?: 'primary' | 'secondary' | 'red',
      expand?: boolean,
      disabled?: boolean,
      onClick: Function,
    }>(),
    {
      variant: 'primary',
      expand: false,
      disabled: false,
    });

const buttonRef = ref<HTMLButtonElement | null>(null);
const slot1 = ref<HTMLButtonElement | null>(null);
const slot2 = ref<HTMLButtonElement | null>(null);

let ctx: gsap.Context | null = null
onMounted(() => {
  ctx = gsap.context(() => {

    var hover_animation_tl = gsap.timeline({paused: true});
    hover_animation_tl.to(
        slot1.value,
        {
          transform: 'translateY(-150%)',
          opacity: 0,
          duration: .3,
          ease: "power2.easeInOut",
        }
    );
    hover_animation_tl.to(
        slot2.value,
        {
          display: 'block',
          opacity: 1,
          transform: 'translate(-50%, -50%)',
          duration: .3,
          delay: -.2,
          ease: "power2.easeInOut",
        }
    )

    Observer.create({
      target: buttonRef.value,
      onHover: () => {
        if (!props.disabled) {
          hover_animation_tl.play()
        }
      },
      onHoverEnd: () => hover_animation_tl.reverse(),
    });
  }, this);
})

onBeforeUnmount(() => ctx?.revert())
</script>

<template>
  <button ref="buttonRef" :class="[props.variant, {'expand': props.expand}, {'disabled': props.disabled}]"
          @click="onClick" :disabled="props.disabled">
    <span ref="slot1"><slot/></span>
    <span ref="slot2"><slot/></span>
  </button>
</template>

<style scoped>
button {
  opacity: 1;
  display: block;
  position: relative;
  min-width: 150px;
  border-radius: var(--m-border-radius);
  letter-spacing: 3px;
  font-weight: 700;
  text-transform: none;
  font-size: var(--s-font-size);
  border: none;
  text-overflow: ellipsis;
  padding: var(--s-spacing) var(--m-spacing);
  cursor: pointer;
}

button.disabled {
  filter: blur(2px);
  cursor: not-allowed;
}

button span {
  pointer-events: none;
  width: 100%;
  display: block;
}

button span:last-child {
  display: none;
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, 100%);
  opacity: 0;
}

.primary {
  background-color: var(--primary-color);
  color: var(--primary-color-text);
}

.secondary {
  background-color: var(--secondary-color);
  color: var(--secondary-color-text);
}

.red {
  background-color: var(--red-color);
  color: var(--red-color-text);
}

button.expand {
  width: 100%;
}
</style>