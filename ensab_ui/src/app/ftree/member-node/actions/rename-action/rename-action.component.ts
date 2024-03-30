import { NgIf } from '@angular/common';
import { Component, input } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import ActionsGroup from '../../actionsGroup';
import Member from '../../member';

@Component({
  selector: 'rename',
  standalone: true,
  imports: [NgIf,ReactiveFormsModule],
  templateUrl: './rename-action.component.html',
})
export class RenameActionComponent {
  member = input<Member>()
  actions = input<ActionsGroup>()
  name_control = new FormControl('')

  on_submit(event : Event) {
    event.preventDefault()
    const name = this.name_control.value;
    if (!name) {
      return;
    }
    this.member()?.name.set(name)
    this.actions()?.rename_son_done()
  }
}
