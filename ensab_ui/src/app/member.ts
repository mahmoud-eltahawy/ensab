import { signal, WritableSignal } from "@angular/core"

export default class Member {
    id: string;
    name: WritableSignal<String>;
    is_male: boolean;
    sons: WritableSignal<Member[]>;

    constructor(name : string) {
      this.id = crypto.randomUUID();
      this.name = signal(name);
      this.is_male = true;
      this.sons = signal([])
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
