<template>
  <div class="hello">
    <div v-for="(event, index) in trace.events" :key="index">
      {{ event.event }}
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { StructuredTrace, getTrace } from '@/data'

export default defineComponent({
  name: 'HelloWorld',
  data () {
    return {
      trace: {
        events: []
      } as StructuredTrace
    }
  },
  mounted () {
    getTrace().then((t) => {
      this.trace = t
      console.log(this.trace)
    })
  }
})
</script>

<style scoped lang="scss">
</style>
