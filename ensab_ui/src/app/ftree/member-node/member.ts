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
  origin : RawMember;
  copy : Member;

  constructor(member : Member) {
    this.origin = member.raw()
    this.copy = member;
  }

  private updates(): SonlessRawMember[] {
    const origin = this.origin;
    const copy = this.copy.raw();

    function compare(origin: RawMember, copy: RawMember): SonlessRawMember[] {
        let first : SonlessRawMember | undefined = undefined;
        let rest : SonlessRawMember[] = []
        if (origin.name != copy.name || origin.is_male != copy.is_male) {
            first = {id : copy.id,name : copy.name,is_male : copy.is_male}
        };
        const copy_sons_ids = copy.sons.map(x => x.id);
        const origin_sons = origin
            .sons
            .filter((x) => copy_sons_ids.includes(x.id));
        for (const origin_son of origin_sons) {
          const copy_son = copy.sons.find(x => x.id === origin_son.id)
          if (copy_son) {
            rest = rest.concat(compare(origin_son,copy_son))
          }
        }
        if (first) {
          return [...rest,first]
        } else {
          return rest
        }
    }
    return compare(origin, copy)
  }

  private created():[string,RawMember[]][] {
    const origin = this.origin;
    const copy = this.copy.raw();

    function compare(origin: RawMember, copy: RawMember) : [string,RawMember[]][] {
      const origin_sons_ids = origin.sons.map(x => x.id);
      const copy_sons = copy
          .sons
          .filter(son => !origin_sons_ids.includes(son.id));
      let first : [string,RawMember[]] | undefined= undefined 
      if (copy_sons.length > 0) {
        first = [copy.id,copy_sons]
      }

      let rest : [string,RawMember[]][] = []
      for(const origin_son of origin.sons) {
        const copy_son = copy.sons.find(x => x.id === origin_son.id)
        if (copy_son) {
          rest = rest.concat(compare(origin_son,copy_son))
        }
      }
      if (first) {
        return [...rest,first];
      } else {
        return rest;
      }
    }

    return compare(origin, copy)
  }

  private deleted():string[] {
    const origin = this.origin;
    const copy = this.copy.raw();

    function compare(origin: RawMember, copy: RawMember) : RawMember[] {
      if (!copy || !origin) {
        return []
      }
      const copy_sons_ids = copy.sons.map(x => x.id);
      let first = origin
          .sons
          .filter(son => !copy_sons_ids.includes(son.id))

      let rest : RawMember[] = []
      for(const origin_son of origin.sons) {
        const copy_son = copy.sons.find(x => x.id === origin_son.id)
        if (copy_son) {
          rest = rest.concat(compare(origin_son,copy_son))
        }
      }
      return [...rest,...first];
    }

    return compare(origin, copy).map(x => x.id)
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
    // this.http.post("http://localhost:8080/member", member).subscribe();
  }

}

type Action = 'Preview'|'Add'|'Remove'|'Update'

export default class Member {
  private static instance: Member;
  private static waitlist : WritableSignal<string[]> = signal([])
  static updates : Updates;
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
    Member.waitlist.update(xs => [...xs,this.id]);
  }

  checkAction(): boolean {
    const list = Member.waitlist()
    return list.at(list.length -1) === this.id
  }

  private PreviewAction() {
    this.action.set('Preview')
  }

  redrawAction() {
    Member.waitlist.update(xs => xs.filter(x => x !== this.id));
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
      Member.updates = new Updates(Member.instance)
    }
    return Member.instance;
  }

  static getInstanceFromRaw({ id, name, is_male, sons }: RawMember): Member {
    Member.instance =  new Member(name, id, is_male, sons);
    Member.updates = new Updates(Member.instance);
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
    }
  }
}
