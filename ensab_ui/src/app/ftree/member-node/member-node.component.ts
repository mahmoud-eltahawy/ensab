import { Component, model } from '@angular/core';
import Member from './member';
import { NgFor, NgIf } from '@angular/common';
import ActionsGroup from './actionsGroup';
import { ActionsButtonsFormComponent } from './actions-buttons-form/actions-buttons-form.component';
import { AddSonActionComponent } from './actions/add-son-action/add-son-action.component';
import { RemoveSonActionComponent } from './actions/remove-son-action/remove-son-action.component';
import { RenameActionComponent } from './actions/rename-action/rename-action.component';

@Component({
  selector: 'member-node',
  standalone: true,
  imports: [NgIf,NgFor,ActionsButtonsFormComponent,AddSonActionComponent,RemoveSonActionComponent,RenameActionComponent],
  templateUrl: './member-node.component.html',
})
export class MemberNodeComponent {
  member = model<Member>();
  actions = new ActionsGroup();
}
