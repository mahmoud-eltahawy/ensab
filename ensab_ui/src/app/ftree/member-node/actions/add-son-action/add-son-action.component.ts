import { NgIf } from '@angular/common';
import { Component, input, signal } from '@angular/core';
import ActionsGroup from '../../actionsGroup';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import Member from '../../member';

@Component({
  selector: 'add-son',
  standalone: true,
  imports: [NgIf,ReactiveFormsModule],
  templateUrl: './add-son-action.component.html',
})
export class AddSonActionComponent {
  name = new FormControl('')
  member = input<Member>()

  actions = input<ActionsGroup>()
  on_submit(event: Event) {
    event.preventDefault()
    const names = this.name.value?.split(',')
    if(names) {
      for (const name of names) {
        this.member()?.add_son(name,true)
      }
    }
    this.name.setValue("")
    this.actions()?.add_son_done()
  }

  is_only = signal(true)
  on_input(){
    const value = this.name.value;
    if (value?.includes(',')) {
      this.is_only.set(false)
    } else {
      this.is_only.set(true)
    }
  }
}
