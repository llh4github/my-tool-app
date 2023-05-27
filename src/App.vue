<script setup lang="ts">
import {NInput} from "naive-ui"

const value = ref("")
</script>

<template>
    <div>

        <n-input
                v-model:value="value"
                type="textarea"
                placeholder="基本的 Textarea"
        />
    </div>
</template>
