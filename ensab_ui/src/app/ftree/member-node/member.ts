import { HttpClient } from "@angular/common/http";
import { signal, WritableSignal } from "@angular/core";
import { url } from "../../shared";

export type RawMember = {
  id: string;
  name: string;
  is_male: boolean;
  sons: RawMember[];
};

type SonlessRawMember = {
  id: string;
  name: string;
  is_male: boolean;
};

class Updates {
  created: WritableSignal<Map<string, RawMember[]>>;
  updates: WritableSignal<SonlessRawMember[]>;
  deleted: WritableSignal<Set<string>>;

  constructor() {
    this.created = signal(new Map());
    this.updates = signal([]);
    this.deleted = signal(new Set());
  }

  private clear() {
    this.created.set(new Map());
    this.updates.set([]);
    this.deleted.set(new Set());
  }

  is_dirty(): boolean {
    return this.created().size !== 0 || this.updates().length !== 0 ||
      this.deleted().size !== 0;
  }

  commit(http: HttpClient) {
    for (const [parent_id, sons] of this.created()) {
      http.post(url(`member/${parent_id}`), sons).subscribe();
    }
    if (this.updates().length > 0) {
      http.put(url("member"), this.updates()).subscribe();
    }
    for (const id of this.deleted()) {
      http.delete(url(`member/${id}`)).subscribe();
    }
    this.clear();
  }

  record_update(member: SonlessRawMember): void {
    const old_member = this.updates().find((x) => x.id === member.id);
    if (old_member) {
      old_member.name = member.name;
      old_member.is_male = member.is_male;
    } else {
      this.updates.update((xs) => [...xs, member]);
    }
    // TODO : check if the fields are matching original member and if so delete the update
  }

  record_create(parent_id: string, member: RawMember): void {
    this.created.update((xs) => {
      const old_parent_sons = xs.get(parent_id);
      if (old_parent_sons) {
        const new_sons = [...old_parent_sons, member];
        const arr = Array.from(xs.entries());
        return new Map([...arr, [parent_id, new_sons]]);
      } else {
        const arr = Array.from(xs.entries());
        return new Map([...arr, [parent_id, [member]]]);
      }
    });
  }

  record_delete(id: string) {
    if (this.deleted().has(id)) {
      this.deleted.update((xs) => new Set([...xs].filter((x) => x !== id)));
    } else {
      this.deleted.update((xs) => new Set([...xs, id]));
    }
  }
}

export default class Member {
  private static instance: Member;

  static updates = new Updates();
  actions: ActionsGroup;
  id: string;
  name: WritableSignal<string>;
  is_male: boolean;
  sons: WritableSignal<Member[]>;

  private constructor(
    name: string,
    id = crypto.randomUUID() as string,
    is_male = true,
    sons = [] as RawMember[],
  ) {
    this.id = id;
    this.name = signal(name);
    this.is_male = is_male;
    this.actions = new ActionsGroup();
    if (sons.length === 0) {
      this.sons = signal([]);
    } else {
      this.sons = signal(
        sons.map((x) => new Member(x.name, x.id, x.is_male, x.sons)),
      );
    }
  }

  static getInstance(name: string | undefined = undefined): Member {
    if (!this.instance && name) {
      Member.instance = new Member(name);
    }
    return Member.instance;
  }

  static getInstanceFromRaw({ id, name, is_male, sons }: RawMember): Member {
    return new Member(name, id, is_male, sons);
  }

  raw(): RawMember {
    const result: RawMember = {
      id: this.id,
      name: this.name(),
      is_male: this.is_male,
      sons: this.sons().map((x) => x.raw()),
    } as const;
    return result;
  }
  sonless_raw(): SonlessRawMember {
    const result: SonlessRawMember = {
      id: this.id,
      name: this.name(),
      is_male: this.is_male,
    } as const;
    return result;
  }

  with_sons(names: string[]): void {
    const name = names.pop();
    if (!name) {
      return;
    }
    const son = new Member(name);
    son.with_sons(names);
    this.sons = signal([son]);
  }

  static create_from_name(name: string): Member {
    const names = name.split(" ");
    const namei = names.pop();
    if (!namei) {
      return new Member("");
    }
    const person = new Member(namei);
    person.with_sons(names);
    return person;
  }

  add_son(name: string, is_male: boolean) {
    const person_from_name = Member.create_from_name(name);
    Member.updates.record_create(this.id, person_from_name.raw());
    this.sons.update((sons) => {
      const son_with_similar_name = sons.find((x) =>
        x.name() == person_from_name.name()
      );
      if (son_with_similar_name) {
        const similar_sons = son_with_similar_name.sons();
        son_with_similar_name.sons.update((sons) => {
          for (const person of similar_sons) {
            person.is_male = is_male;
          }
          return [...sons, ...similar_sons];
        });
        return [];
      } else {
        person_from_name.is_male = is_male;
        return [...sons, person_from_name];
      }
    });
  }

  remove_son_toggle(id: string) {
    Member.updates.record_delete(id);
  }
}

class ActionsGroup {
  take_action: WritableSignal<boolean>;
  add_son: WritableSignal<boolean>;
  remove_son: WritableSignal<boolean>;
  rename_son: WritableSignal<boolean>;

  constructor() {
    this.take_action = signal(false);
    this.add_son = signal(false);
    this.remove_son = signal(false);
    this.rename_son = signal(false);
  }

  toggle_action() {
    this.take_action.update((x) => !x);
  }

  add_son_action() {
    this.add_son.set(true);
    this.toggle_action();
  }

  add_son_done() {
    this.add_son.set(false);
  }

  remove_son_action() {
    this.remove_son.set(true);
    this.toggle_action();
  }

  remove_son_done() {
    this.remove_son.set(false);
  }

  rename_son_action() {
    this.rename_son.set(true);
    this.toggle_action();
  }

  rename_son_done() {
    this.rename_son.set(false);
  }
}
