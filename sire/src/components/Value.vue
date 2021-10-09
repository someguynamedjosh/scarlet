<template>
  <div class="grey" @click="toggleExpanded">
    {{ label }}: p{{ value.pool_id }} i{{ value.index }}
  </div>
  <div v-if="expanded">
    <template v-if="'BuiltinOperation' in got">
      <div class="green">BuiltinOperation</div>
    </template>
    <template v-if="'BuiltinValue' in got">
      <div class="green">BuiltinValue</div>
      <div>{{ got.BuiltinValue }}</div>
    </template>
    <template v-if="'From' in got">
      <div class="green">From</div>
      <div>
        variable: p{{ got.From.variable.pool_id }} i{{
          got.From.variable.index
        }}
      </div>
      <div>
        <Value label="base" :values="values" :value="got.From.base" />
      </div>
    </template>
    <template v-if="'Match' in got">
      <div class="green">Match</div>
      <div class="indented">
        <div>
          <Value label="base" :values="values" :value="got.Match.base" />
        </div>
        <div class="spacer" />
        <template v-for="(casee, index) of got.Match.cases" :key="index">
          <div>
            <Value label="on" :values="values" :value="casee[0]" />
          </div>
          <div>
            <Value label="is" :values="values" :value="casee[1]" />
          </div>
          <div class="spacer" />
        </template>
      </div>
    </template>
    <template v-if="'Opaque' in got">
      <div class="green">Opaque</div>
      <div>class: {{ got.Opaque.class }}</div>
      <div>id: p{{ got.Opaque.id.pool_id }} i{{ got.Opaque.id.index }}</div>
    </template>
    <template v-if="'Substituting' in got">
      <div class="green">Substituting</div>
      <div>
        <Value label="base" :values="values" :value="got.Substituting.base" />
      </div>
      <div>target: p{{ got.Substituting.target.pool_id }} i{{ got.Substituting.target.index }}</div>
      <div>
        <Value label="value" :values="values" :value="got.Substituting.value" />
      </div>
    </template>
    <div class="spacer" />
  </div>
</template>

<script lang="ts">
import { defineComponent, Prop } from 'vue'
import { AnnotatedValue, Id, Pool, StructuredEvent, Value } from '@/data'

export default defineComponent({
  name: 'Value',
  props: {
    label: String,
    values: {
      type: Object
    } as Prop<Pool<AnnotatedValue>>,
    value: {
      type: Object
    } as Prop<Id>
  },
  computed: {
    got () {
      return this.values?.items?.[this.value!.index]?.value
    }
  },
  data: function () {
    return {
      expanded: false
    }
  },
  methods: {
    toggleExpanded () {
      this.expanded = this.expanded !== true
    }
  }
})
</script>

<style scoped lang="scss">
.indented {
  margin-left: 10px;
  min-width: max-content;
}
div {
  padding: 0;
  user-select: none;
}
div.grey {
  cursor: pointer;
}
</style>
