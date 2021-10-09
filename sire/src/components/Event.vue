<template>
  <div>
    {{ event.fn_name }} {{ event.args }}
    <div class="indented">
        <div v-for="(event, index) in event.body" :key="index">
        <Event :event="event" />
        </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, Prop } from 'vue'
import { StructuredEvent } from '@/data'

export default defineComponent({
  name: 'HelloWorld',
  props: {
    event: {
      type: Object
    } as Prop<StructuredEvent>
  }
})
</script>

<style scoped lang="scss">
.indented {
    margin-left: 15px;
    min-width: max-content;
}
</style>
