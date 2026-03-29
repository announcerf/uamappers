<script setup lang="ts">
import { Typography } from '../../typography';
import ArrowsCounterClockwiseIcon from '@/shared/icons/arrows/ArrowsCounterClockwiseIcon.vue';
import GithubLogoIcon from '@/shared/icons/other/GithubLogoIcon.vue';
import TranslateIcon from '@/shared/icons/other/TranslateIcon.vue';
import type { NavbarProps } from '@/shared/ui/navbar';
import { useNavbar } from '@/shared/ui/navbar';

const props = withDefaults(defineProps<NavbarProps>(), {});

const labels = {
  navbar: {
    community: 'Community',
    rankings: 'Rankings',
    analytics: 'Analytics',
    about: 'About',
  },
};
const { className, sideClassName, actionClassname } = useNavbar(props);
</script>

<template>
  <nav :class="className">
    <div :class="sideClassName" class="gap-6">
      <a href="/">
        <Typography size="sm" weight="medium"> ua.mappers </Typography>
      </a>
      <span class="bg-outline h-4 w-px"></span>
      <a v-for="(value, key) in labels.navbar" :key="key" :href="`/${key}`">
        <Typography size="sm" weight="medium" :class="actionClassname">
          {{ value }}
        </Typography>
      </a>
    </div>
    <div :class="sideClassName" class="gap-4">
      <ArrowsCounterClockwiseIcon :class="actionClassname" />
      <TranslateIcon :class="actionClassname" />
      <GithubLogoIcon :class="actionClassname" />
    </div>
  </nav>
</template>
