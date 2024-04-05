import { ComponentFixture, TestBed } from '@angular/core/testing';
import { NoteUiComponent } from './note-ui.component';

describe('NoteUiComponent', () => {
  let component: NoteUiComponent;
  let fixture: ComponentFixture<NoteUiComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [NoteUiComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(NoteUiComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
