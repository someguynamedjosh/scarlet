<template>
  <div class="hello">
    <div v-for="(event, index) in trace.events" :key="index">
      <Event :event="event" />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { StructuredTrace, getTrace } from '@/data'
import Event from './Event.vue'

export default defineComponent({
  components: { Event },
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
.hello {
  overflow: scroll;
  max-height: 80vh;
}
</style>
