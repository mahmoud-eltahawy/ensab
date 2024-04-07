import { NgIf } from '@angular/common';
import { Component, input, signal } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import Member from '../../member';
import { ActionComponent } from '../action/action.component';

@Component({
  selector: 'add-son',
  standalone: true,
  imports: [NgIf,ReactiveFormsModule,ActionComponent],
  templateUrl: './add-son-action.component.html',
})
export class AddSonActionComponent {
  name = new FormControl('')
  is_male = new FormControl('1');
  member = input<Member>()

  on_submit() {
    const names = this.name.value?.split(',')
    if(!names || names[0] === '') {
      return;
    }
    const is_male = Boolean(this.is_male.value)
    for (const name of names) {
      this.member()?.add_son(name,is_male)
    }
    this.name.setValue("")
    const sons = this.member()!.sons().map(x => x.raw())
    for(const son of sons) {
      Member.updates.record_create(this.member()!.id,son)
    }
    console.log(Member.updates.created())
    this.member()?.actions.add_son_done()
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
