import { NgIf } from '@angular/common';
import { Component, computed, input } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import Member from '../../member';
import { ActionComponent } from '../action/action.component';

@Component({
  selector: 'rename',
  standalone: true,
  imports: [NgIf,ReactiveFormsModule,ActionComponent],
  templateUrl: './rename-action.component.html',
})
export class RenameActionComponent {
  member = input<Member>()
  name_control = new FormControl('')

  actions = computed(() => this.member()?.getActions())

  on_submit() {
    const name = this.name_control.value;
    if (!name) {
      return;
    }
    this.member()?.name.set(name)
    this.actions()?.rename_son_done()
  }
}
