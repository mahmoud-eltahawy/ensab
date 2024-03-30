import { NgIf } from '@angular/common';
import { Component, input, output } from '@angular/core';

@Component({
  selector: 'action',
  standalone: true,
  imports: [NgIf],
  templateUrl: './action.component.html',
})
export class ActionComponent {
  cond = input<boolean>()
  submit = output()
  cancel = output()

  on_submit(event : SubmitEvent) {
    event.preventDefault()
    this.submit.emit()
  }

  on_cancel() {
    this.cancel.emit()
  }
}
