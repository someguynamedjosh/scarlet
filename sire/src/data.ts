/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable camelcase */
export type InputEvent =
    | {
        event: 'enter';
        fn_name: string;
        args: unknown;
    }
    | {
        event: 'leave';
        fn_name: string;
    }

export type StructuredEvent =
    | {
        event: 'call';
        fn_name: string;
        args: unknown;
        body: Array<StructuredEvent>;
    };

export type Id = { pool_id: number; index: number; }
export type Value =
| { BuiltinOperation: '' }
| { BuiltinValue: 'OriginType' }
| { From: { base: Id; variable: Id }}
| { Match: { base: Id; cases: Array<[Id, Id]> }}
| { Opaque: { class: 'Variable' | 'Variant', id: Id, typee: Id }}
| { Substituting: { base: Id; target: Id, value: Id }}
export type AnnotatedValue = {
  cached_type: Id | null;
  cached_reduction: Id | null;
  value: Value;
}
export type Pool<T> = {id: number, items: Array<T>}
export class Environment {
  values: Map<Id, Value> = new Map()
}

function unflattenEvents (source: Array<InputEvent>): Array<StructuredEvent> {
  const c = {
    source,
    index: 0
  }
  const nextEvent = (): StructuredEvent | null => {
    if (c.index >= c.source.length) {
      return null
    } else {
      const e = c.source[c.index]
      c.index += 1
      if (e.event === 'leave') {
        return null
      } else {
        const body = []
        while (true) {
          const next = nextEvent()
          if (next === null) {
            break
          } else {
            body.push(next)
          }
        }
        return {
          event: 'call',
          fn_name: e.fn_name,
          args: e.args,
          body
        }
      }
    }
  }
  const body = []
  while (true) {
    const e = nextEvent()
    if (e === null) {
      return body
    } else {
      body.push(e)
    }
  }
}

interface InputTrace {
    events: Array<InputEvent>;
    stage3: {
      values: Pool<Value>
    }
}

export interface StructuredTrace {
    events: Array<StructuredEvent>;
    stage3: {
      values: Pool<Value>
    }
}

export async function getTrace (): Promise<StructuredTrace> {
  const trace = await fetch('http://localhost:8000/1000.sir')
  const traceData: InputTrace = await trace.json()
  return {
    events: unflattenEvents(traceData.events),
    stage3: {
      values: traceData.stage3.values
    }
  }
}
