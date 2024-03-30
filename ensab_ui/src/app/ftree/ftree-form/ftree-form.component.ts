import { Component } from '@angular/core';
import { FormControl, ReactiveFormsModule } from '@angular/forms';

@Component({
  selector: 'app-ftree-form',
  standalone: true,
  imports: [ReactiveFormsModule],
  templateUrl: './ftree-form.component.html',
})
export class FtreeFormComponent {
    name = new FormControl("");

    onSubmit(event : Event){
      event.preventDefault();
      this.to_tree_page()
    }

    onInput() {
      const value = this.name.value;
      if(value?.trimStart()?.includes(' ')) {
        this.to_tree_page()
      }
    }

    private to_tree_page() {
      const name = this.name.value?.trim();
      if (name?.length !== 0) {
        location.href = `ftree/${name}`
      }
    }
}
