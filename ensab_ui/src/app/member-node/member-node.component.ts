import { Component, input, model, signal } from '@angular/core';
import Member from '../member';
import { NgFor, NgIf } from '@angular/common';

@Component({
  selector: 'member-node',
  standalone: true,
  imports: [NgIf,NgFor],
  templateUrl: './member-node.component.html',
})
export class MemberNodeComponent {
  member = model<Member>();
  take_action = signal(false)
  toggle_action() {
    this.take_action.update(x => !x);  
  } 

  add_son_action(){
    this.member.update(x => {
      if (!x) {
        return x;
      }
      const new_sons = x.sons();
      new_sons.push(new Member("hello"));
      return {...x,sons : signal(new_sons)} as Member;
    });
    this.toggle_action()
  }
  remove_son_action(){
    this.toggle_action()
  }
  rename_son_action(){
    this.toggle_action()
  }
}
