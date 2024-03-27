import { NgIf } from '@angular/common';
import { Component, input, signal } from '@angular/core';
import FtreeSignals from '../../../ftreeSignals';

@Component({
  selector: 'add-son',
  standalone: true,
  imports: [NgIf],
  templateUrl: './add-son-action.component.html',
})
export class AddSonActionComponent {
  actions = input<FtreeSignals>()
  submit_add_son($event: Event) {
    event?.preventDefault()

    this.actions()?.add_son_done()
  }

  is_only = signal(true)
}
