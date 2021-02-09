import Vue, { VueConstructor } from 'vue';
import { Store } from 'vuex';
import { AppState } from '@/app/store';
import { UsersService } from '@/domain/users/service';
import { NamespacesService } from '@/domain/namespaces/service';
import { OperationsService } from '@/domain/operations/service';
import { ToolsService } from '@/domain/tools/service';
import { KernelService } from '@/domain/kernel/service';
import { InboxService } from '@/domain/inbox/service';
import { AnalyticsService } from '@/domain/analytics/service';
import { FilesService } from '@/domain/files/service';
import { NewsletterService } from '@/domain/newsletter/service';
import { CalendarService } from '@/domain/calendar/service';

// export abstract class VueApp extends Vue {
//   public $store!: Store<AppState>;
//   public $usersService!: UsersService;
//   public $groupsService!: GroupsService;
//   public $projectsService!: ProjectsService;
//   public $namespacesService!: NamespacesService;
// }
abstract class VueAppClass extends Vue {
  public $store!: Store<AppState>;
  public $kernelService!: KernelService;
  public $inboxService!: InboxService;
  public $usersService!: UsersService;
  public $namespacesService!: NamespacesService;
  public $operationsService!: OperationsService;
  public $toolsService!: ToolsService;
  public $analyticsService!: AnalyticsService;
  public $filesService!: FilesService;
  public $newsletterService!: NewsletterService;
  public $calendarService!: CalendarService;
}
export const VueApp = Vue as VueConstructor<VueAppClass>;
