import { NgIf } from '@angular/common';
import { Component, input } from '@angular/core';
import Member from '../../../member';
import FtreeSignals from '../../ftreeSignals';

@Component({
  selector: 'actions-buttons',
  standalone: true,
  imports: [NgIf],
  templateUrl: './actions-buttons-form.component.html',
})
export class ActionsButtonsFormComponent {
  member = input<Member>();
  actions = input<FtreeSignals>()

  // add_son_action(){
  //   this.member.update(x => {
  //     if (!x) {
  //       return x;
  //     }
  //     const new_sons = x.sons();
  //     new_sons.push(new Member("hello"));
  //     return {...x,sons : signal(new_sons)} as Member;
  //   });
  //   this.actions()?.add_son_action()
  // }
}
