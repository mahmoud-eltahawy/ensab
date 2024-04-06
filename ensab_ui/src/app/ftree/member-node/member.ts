import { signal, WritableSignal } from "@angular/core"

type RawMember = {
  id : string;
  name : string;
  is_male : boolean,
  sons : RawMember[]
}

type SonlessRawMember = {
  id : string;
  name : string;
  is_male : boolean,
}

export default class Member {
    private static instance : Member;

    static updates : WritableSignal<SonlessRawMember[]> = signal([]); 
    actions : ActionsGroup;  
    id : string;
    name: WritableSignal<string>;
    is_male: boolean;
    sons: WritableSignal<Member[]>;

    private constructor(name : string,id = crypto.randomUUID() as string,is_male = true,sons = [] as RawMember[]) {
      this.id = id;
      this.name = signal(name);
      this.is_male = is_male;
      this.actions = new ActionsGroup()
      if(sons.length === 0) {
        this.sons = signal([])
      } else {
        this.sons = signal(sons.map(x => new Member(x.name,x.id,x.is_male,x.sons)))
      }
    }

    static record_member_update(member : SonlessRawMember) {
      const old_member = Member.updates().find(x => x.id === member.id);
      if(old_member) {
        old_member.name = member.name; 
        old_member.is_male = member.is_male;
      } else {
        Member.updates.update(xs => [...xs,member])
      }
      // TODO : check if the fields are matching original member and if so delete the update
      console.log(Member.updates())
    }

    static get_and_clear_updates(){
      const updates =  Member.updates();
      Member.updates.set([])
      return updates;
    }

    static getInstance(name : string | undefined = undefined): Member {
      if (!this.instance && name) {
        Member.instance =  new Member(name)
      }
      return Member.instance
    }

    static getInstanceFromRaw({id,name,is_male,sons} : RawMember) : Member {
      return new Member(name,id,is_male,sons);
    }

    raw() : RawMember {
      const result : RawMember = {
        id : this.id,
        name : this.name(),
        is_male : this.is_male,
        sons : this.sons().map(x => x.raw())
      } as const;
      return result;
    }
    sonless_raw() : SonlessRawMember {
      const result : SonlessRawMember = {
        id : this.id,
        name : this.name(),
        is_male : this.is_male,
      } as const;
      return result;
    }

    with_sons(names: string[]): void {
        const name = names.pop(); 
        if(!name){
          return;      
        }
        const son = new Member(name);
        son.with_sons(names);
        this.sons = signal([son]);
    }

    static create_from_name(name: string): Member {
        const names = name.split(' ')
        const namei = names.pop(); 
        if(!namei){
          return new Member("")      
        }
        const person = new Member(namei);
        person.with_sons(names);
        return person;
    }

    add_son(name: string, is_male: boolean) {
        const person_from_name = Member.create_from_name(name);
        this.sons.update(sons => {
            const son_with_similar_name = sons.find(x => x.name() == person_from_name.name());
            if (son_with_similar_name) {
              const similar_sons = son_with_similar_name.sons()
              son_with_similar_name.sons.update(sons => {
                for(const person of similar_sons) {
                  person.is_male = is_male;
                }
                return [...sons,...similar_sons]
              })
              return []
            } else {
              person_from_name.is_male = is_male;
              return [...sons,person_from_name]
            }
        });
    }
}

class ActionsGroup {
    take_action : WritableSignal<boolean>
    add_son : WritableSignal<boolean>
    remove_son : WritableSignal<boolean>
    rename_son : WritableSignal<boolean>

    constructor() {
        this.take_action = signal(false)
        this.add_son = signal(false)
        this.remove_son = signal(false)
        this.rename_son = signal(false)
    }

    toggle_action() {
        this.take_action.update(x => !x);  
    } 

    add_son_action() {
        this.add_son.set(true)
        this.toggle_action()
    }

    add_son_done() {
        this.add_son.set(false)
    }

    remove_son_action() {
        this.remove_son.set(true)
        this.toggle_action()
    }

    remove_son_done() {
        this.remove_son.set(false)
    }

    rename_son_action(){
        this.rename_son.set(true)
        this.toggle_action()
    }

    rename_son_done(){
        this.rename_son.set(false)
    }
}
