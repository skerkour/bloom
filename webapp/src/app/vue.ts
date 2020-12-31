import Vue, { VueConstructor } from 'vue';
import { Store } from 'vuex';
import { AppState } from '@/app/store';
import { UsersService } from '@/domain/users/service';
import { GroupsService } from '@/domain/groups/service';
import { ProjectsService } from '@/domain/projects/service';
import { NamespacesService } from '@/domain/namespaces/service';
import { SupportService } from '@/domain/support/service';
import { CollaborationService } from '@/domain/collaboration/service';
import { GrowthService } from '@/domain/growth/service';
import { OperationsService } from '@/domain/operations/service';
import { ToolsService } from '@/domain/tools/service';
import { KernelService } from '@/domain/kernel/service';
import { BotsService } from '@/domain/bots/service';

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
  public $usersService!: UsersService;
  public $groupsService!: GroupsService;
  public $projectsService!: ProjectsService;
  public $namespacesService!: NamespacesService;
  public $supportService!: SupportService;
  public $collaborationService!: CollaborationService;
  public $growthService!: GrowthService;
  public $operationsService!: OperationsService;
  public $toolsService!: ToolsService;
  public $botsService!: BotsService;
}
export const VueApp = Vue as VueConstructor<VueAppClass>;
