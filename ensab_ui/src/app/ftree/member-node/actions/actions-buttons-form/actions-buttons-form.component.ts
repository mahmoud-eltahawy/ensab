import { Component, input } from '@angular/core';
import Member from '../../member';

@Component({
  selector: 'actions-buttons',
  standalone: true,
  imports: [],
  templateUrl: './actions-buttons-form.component.html',
})
export class ActionsButtonsFormComponent {
  member = input<Member>();
}
