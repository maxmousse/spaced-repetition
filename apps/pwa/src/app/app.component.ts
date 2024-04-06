import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { map } from 'rxjs';
import { GetUsersQuery } from './get-users.graphql';
import { SidebarModule } from 'primeng/sidebar';
import { MenubarModule } from 'primeng/menubar';

@Component({
  standalone: true,
  imports: [RouterModule, CommonModule, SidebarModule, MenubarModule],
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss',
})
export class AppComponent {
  title = 'pwa';
  users$ = this.getUsersQuery.fetch().pipe(map((result) => result.data));

  constructor(private getUsersQuery: GetUsersQuery) {}
}
