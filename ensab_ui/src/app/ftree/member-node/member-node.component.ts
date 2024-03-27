import { Component, model, signal } from '@angular/core';
import Member from '../../member';
import { NgFor, NgIf } from '@angular/common';
import ActionsGroup from './actionsGroup';
import { ActionsButtonsFormComponent } from './actions-buttons-form/actions-buttons-form.component';
import { AddSonActionComponent } from './actions/add-son-action/add-son-action.component';

@Component({
  selector: 'member-node',
  standalone: true,
  imports: [NgIf,NgFor,ActionsButtonsFormComponent,AddSonActionComponent],
  templateUrl: './member-node.component.html',
})
export class MemberNodeComponent {
  member = model<Member>();
  actions = new ActionsGroup();
}
