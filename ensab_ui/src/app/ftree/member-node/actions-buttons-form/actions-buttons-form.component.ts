import { NgIf } from '@angular/common';
import { Component, input } from '@angular/core';
import Member from '../../../member';
import ActionsGroup from '../actionsGroup';

@Component({
  selector: 'actions-buttons',
  standalone: true,
  imports: [NgIf],
  templateUrl: './actions-buttons-form.component.html',
})
export class ActionsButtonsFormComponent {
  member = input<Member>();
  actions = input<ActionsGroup>()

}
