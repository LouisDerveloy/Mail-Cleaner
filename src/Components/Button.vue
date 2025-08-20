<script lang="ts" setup>
import gsap from 'gsap';
import {Observer} from 'gsap/Observer';
import {onBeforeUnmount, onMounted, ref} from "vue";

gsap.registerPlugin(Observer)

// TODO: Add an option to have an icon in the button.
const props = withDefaults(
    defineProps<{
      variant?: 'primary' | 'secondary' | 'red',
      expand?: boolean,
      disabled?: boolean,
      onClick: Function,
      size?: 'small' | 'medium' | 'large',
      icon?: string,
      iconAlt?: string,
    }>(),
    {
      variant: 'primary',
      expand: false,
      disabled: false,
      size: 'medium',
      icon: '',
      iconAlt: '',
    });

const buttonRef = ref<HTMLButtonElement | null>(null);
const slot1 = ref<HTMLButtonElement | null>(null);
const slot2 = ref<HTMLButtonElement | null>(null);

let ctx: gsap.Context | null = null
onMounted(() => ctx = gsap.context(() => {

  if (buttonRef.value === null || slot1.value === null || slot2.value === null) return;

  buttonRef.value.addEventListener('mousedown', () => gsap.to(buttonRef.value, {
    scale: 0.8,
    duration: 0.1,
    ease: "power1.out"
  }));
  const releaseAnimation = () => gsap.to(buttonRef.value, {scale: 1, duration: 0.1, ease: "power1.out"});
  buttonRef.value.addEventListener('mouseup', releaseAnimation);
  buttonRef.value.addEventListener('mouseleave', releaseAnimation);

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
        display: 'flex',
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
}, this))

onBeforeUnmount(() => ctx?.revert())
</script>

<template>
  <button ref="buttonRef" :class="[props.variant, {'expand': props.expand}, {'disabled': props.disabled}, props.size]"
          :disabled="props.disabled" @click="onClick">
    <span ref="slot1"><slot/><img v-if="icon !== ''" :alt="iconAlt" :src="icon"></span>
    <span ref="slot2"><slot/><img v-if="icon !== ''" :alt="iconAlt" :src="icon"></span>
  </button>
</template>

<style scoped>
button {
  opacity: 1;
  display: block;
  position: relative;
  text-transform: none;
  border: none;
  text-overflow: ellipsis;
  cursor: pointer;
  overflow: hidden;
}


button.small {
  min-width: 32px;
  border-radius: var(--s-border-radius);
  padding: var(--xs-spacing) var(--s-spacing);
  letter-spacing: 1px;
  font-size: var(--s-font-size);
}

button.small img {
  max-height: 16px;
}

button.medium {
  min-width: 100px;
  border-radius: var(--m-border-radius);
  padding: var(--s-spacing) var(--m-spacing);
  letter-spacing: 3px;
  font-size: var(--s-font-size);
}

button.medium img {
  max-height: 32px;
}

button.large {
  min-width: 200px;
  border-radius: var(--l-border-radius);
  padding: var(--m-spacing) var(--l-spacing);
  letter-spacing: 3px;
  font-size: var(--l-font-size);
}

button.large img {
  max-height: 64px;
}

button.disabled {
  filter: blur(2px) grayscale(0.8);
  cursor: not-allowed;
}

button span {
  font-weight: 700;
  pointer-events: none;
  width: 100%;
  display: flex;
  flex-direction: row;
  gap: var(--xs-spacing);
  justify-content: center;
  align-items: center;
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