import { Component, input, signal } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';
import Member from '../../member';
import { ActionComponent } from '../action/action.component';

type FCS = FormControl<string | null>;
function extract_values(name : FCS,is_male : FCS): [string[],boolean] | undefined {
  const names = name.value?.split(',')
  name.setValue("")
  if(!names || names[0] === '') {
    return undefined;
  }
  const ismale = Boolean(is_male.value)
  is_male.setValue('1')

  return [names,ismale]
}

@Component({
  selector: 'add-son',
  standalone: true,
  imports: [ReactiveFormsModule,ActionComponent],
  templateUrl: './add-son-action.component.html',
})
export class AddSonActionComponent {
  name = new FormControl('')
  is_male = new FormControl('1');
  member = input<Member>()

  on_submit() {
    const values = extract_values(this.name,this.is_male)
    if(!values){ return;}
    const [names,is_male] = values;
    for (const name of names) {
      this.member()?.add_son(name,is_male)
    }
    this.member()?.actions.add_son_done()
  }

  is_only = signal(true)

  on_input(){
    const value = this.name.value;
    if (value?.includes(',')) {
      this.is_only.set(false)
    } else {
      this.is_only.set(true)
    }
  }
}
