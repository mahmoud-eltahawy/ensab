import { Component, input, output } from '@angular/core';

@Component({
  selector: 'action',
  standalone: true,
  imports: [],
  templateUrl: './action.component.html',
})
export class ActionComponent {
  cond = input.required<boolean>()
  submit = output()
  cancel = output()

  on_submit() {
    this.submit.emit()
  }

  on_cancel() {
    this.cancel.emit()
  }
}
