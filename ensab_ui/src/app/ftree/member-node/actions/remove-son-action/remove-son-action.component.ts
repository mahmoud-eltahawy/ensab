import { NgFor, NgIf } from '@angular/common';
import { Component, computed, input, signal } from '@angular/core';
import Member from '../../member';
import { ActionComponent } from '../action/action.component';

@Component({
  selector: 'remove-son',
  standalone: true,
  imports: [NgIf,NgFor,ActionComponent],
  templateUrl: './remove-son-action.component.html',
})
export class RemoveSonActionComponent {
  member = input<Member>()
  removed = signal<string[]>([])

  on_submit() {
    this.member()?.sons.update(xs => this.get_restored())
    this.member()?.actions?.remove_son_done()
  }

  remove(id : string) {
    this.removed.update(xs => [...xs,id])
    this.member()?.remove_son_toggle(id);
  }

  restore(id : string) {
    this.removed.update(xs => xs.filter(x => x !== id))
    this.member()?.remove_son_toggle(id);
  }

  get_removed = computed(() => this.member()?.sons().filter(x => this.removed().includes(x.id)) ?? [])

  get_restored = computed(() => this.member()?.sons().filter(x => !this.removed().includes(x.id)) ?? [])
}
