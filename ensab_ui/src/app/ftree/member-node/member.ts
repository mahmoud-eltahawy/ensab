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
    const old_parent_sons = this.created().get(parent_id);
    if (old_parent_sons) {
      const siblings = old_parent_sons.filter(x => x.name !== member.name)
      this.created.update(xs => {
        const new_sons = [...siblings, member];
        const arr = Array.from(xs.entries());
        return new Map([...arr, [parent_id, new_sons]]);
      })
    } else {
      this.created.update(xs => {
        const arr = Array.from(xs.entries());
        return new Map([...arr, [parent_id, [member]]]);
      })
    }
    console.log(this.created())
  }

  record_delete(id: string) {
    if (this.deleted().has(id)) {
      this.deleted.update((xs) => new Set([...xs].filter((x) => x !== id)));
    } else {
      this.deleted.update((xs) => new Set([...xs, id]));
    }
  }
}

type Action = 'Preview'|'Add'|'Remove'|'Update'

export default class Member {
  private static instance: Member;

  static updates = new Updates();
  private waitlist = new Waitlist()
  action : WritableSignal<Action>
  id: string;
  name: WritableSignal<string>;
  is_male: WritableSignal<boolean>;
  sons: WritableSignal<Member[]>;

  private constructor(
    name: string,
    id = crypto.randomUUID() as string,
    is_male = true,
    sons = [] as RawMember[],
  ) {
    this.id = id;
    this.name = signal(name);
    this.is_male = signal(is_male);
    this.action = signal('Preview')
    if (sons.length === 0) {
      this.sons = signal([]);
    } else {
      this.sons = signal(
        sons.map((x) => new Member(x.name, x.id, x.is_male, x.sons)),
      );
    }
  }

  takeAction() {
    this.waitlist.take(this.id)
  }

  checkAction(): boolean {
    const list = Waitlist.list()
    return list.at(list.length -1) === this.id
  }

  private PreviewAction() {
    this.action.set('Preview')
  }

  redrawAction() {
    this.waitlist.redraw(this.id)
    this.PreviewAction()
  }

  removeAction() {
    this.action.set('Remove')
  }

  updateAction() {
    this.action.set('Update')
  }

  addAction() {
    this.action.set('Add')
  }

  static getInstance(name: string | undefined = undefined): Member {
    if (!this.instance && name) {
      Member.instance = new Member(name);
    }
    return Member.instance;
  }

  static getInstanceFromRaw({ id, name, is_male, sons }: RawMember): Member {
    Member.instance =  new Member(name, id, is_male, sons);
    return Member.instance;
  }

  raw(): RawMember {
    const result: RawMember = {
      id: this.id,
      name: this.name(),
      is_male: this.is_male(),
      sons: this.sons().map((x) => x.raw()),
    } as const;
    return result;
  }
  sonless_raw(): SonlessRawMember {
    const result: SonlessRawMember = {
      id: this.id,
      name: this.name(),
      is_male: this.is_male(),
    } as const;
    return result;
  }

  private with_sons(names: string[]): void {
    const name = names.pop();
    if (!name) {
      return;
    }
    const son = new Member(name);
    son.with_sons(names);
    this.sons = signal([son]);
  }

  static create_from_name(name: string): Member {
    const names = name.split("->");
    const namei = names.pop();
    if (!namei) {
      return new Member("");
    }
    const person = new Member(namei);
    person.with_sons(names);
    return person;
  }

  add_son(member : Member) {
    const sons = this.sons()
    const same_person = sons.find(x => x.name() === member.name()) 
    if(same_person){
      for(const person of member.sons()) {
        same_person.add_son(person)
      } 
    } else {
      this.sons.update(xs => [...xs,member])
      Member.updates.record_create(this.id, member.raw());
    }
  }

  remove_son_toggle(id: string) {
    Member.updates.record_delete(id);
  }
}

class Waitlist {
  static list : WritableSignal<string[]> = signal([])
  take(id : string) {
    Waitlist.list.update(xs => [...xs,id]);
  }

  redraw(id : string) {
    Waitlist.list.update(xs => xs.filter(x => x !== id));
  }
}
