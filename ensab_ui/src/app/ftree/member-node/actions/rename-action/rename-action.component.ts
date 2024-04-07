import { Component, input } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import Member from '../../member';
import { ActionComponent } from '../action/action.component';

@Component({
  selector: 'rename',
  standalone: true,
  imports: [ReactiveFormsModule,ActionComponent],
  templateUrl: './rename-action.component.html',
})
export class RenameActionComponent {
  member = input.required<Member>()
  name_control = new FormControl('')

  on_submit() {
    const name = this.name_control.value;
    if (!name) {
      return;
    }
    this.member().name.set(name)
    this.member().actions.rename_son_done()
    Member.updates.record_update(this.member().sonless_raw())
  }
}
