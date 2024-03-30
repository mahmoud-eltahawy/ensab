import { signal, WritableSignal } from "@angular/core"

export default class Member {
    private static instance : Member;

    private actions : ActionsGroup | undefined  
    id: string;
    name: WritableSignal<string>;
    is_male: boolean;
    sons: WritableSignal<Member[]>;

    private constructor(name : string) {
      this.id = crypto.randomUUID();
      this.name = signal(name);
      this.is_male = true;
      this.sons = signal([])
    }

    getActions() : ActionsGroup {
      if(!this.actions) {
        this.actions = new ActionsGroup()
      }
      return this.actions
    }


    static getInstance(name : string | undefined = undefined): Member {
      if (!this.instance && name) {
        Member.instance =  new Member(name)
      }
      return Member.instance
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
        let person_from_name = Member.create_from_name(name);
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