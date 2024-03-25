import { Routes } from '@angular/router';
import { FtreeFormComponent } from './ftree-form/ftree-form.component';
import { FtreeComponent } from './ftree/ftree.component';
import { HomeComponent } from './home/home.component';

export const routes: Routes = [
  {path :"",component : HomeComponent},
  {path :"ftree",component : FtreeFormComponent},
  {path :"ftree/:name",component : FtreeComponent},
];
