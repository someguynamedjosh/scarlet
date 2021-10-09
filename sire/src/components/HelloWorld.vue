<template>
  <div class="container">
    <div class="events">
      <div v-for="(event, index) in trace.events" :key="index">
        <Event :event="event" />
      </div>
    </div>
    <div class="stage3">
      <div v-for="(value, index) of trace.stage3.values.items" :key="index">
        <Value label="value" :value="{pool_id: trace.stage3.values.id, index}" :values="trace.stage3.values" />
        <div class="spacer" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { StructuredTrace, getTrace } from '@/data'
import Event from './Event.vue'
import Value from './Value.vue'

export default defineComponent({
  components: { Event, Value },
  name: 'HelloWorld',
  data () {
    return {
      trace: {
        events: [],
        stage3: {
          values: {
            id: 0,
            items: []
          }
        }
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
.container {
  max-width: 100vw;
  max-height: 100vh;
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 90vh;
}
.events, .stage3 {
  overflow: scroll;
}
</style>
