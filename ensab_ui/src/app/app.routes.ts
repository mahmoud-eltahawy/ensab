import { Routes } from '@angular/router';
import { FtreeFormComponent } from './ftree/ftree-form/ftree-form.component';
import { FtreeComponent } from './ftree/ftree.component';
import { HomeComponent } from './home/home.component';

export const routes: Routes = [
  {path :"",component : HomeComponent},
  {path :"ftree",title : "انشاء شجرة عائلة",component : FtreeFormComponent},
  {path :"ftree/:name",title : "تفصيل شجرة العائلة",component : FtreeComponent},
];
