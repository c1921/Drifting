<script setup lang="ts">
import type { ColumnDef, SortingState } from "@tanstack/vue-table"
import {
  FlexRender,
  getCoreRowModel,
  getSortedRowModel,
  useVueTable,
} from "@tanstack/vue-table"
import {
  IconArrowDown,
  IconArrowUp,
  IconArrowsUpDown,
  IconLoader2,
} from "@tabler/icons-vue"
import { computed, ref, onMounted } from "vue"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { valueUpdater } from "@/components/ui/table/utils"
import type { ItemData } from "@/types/item"
import { listItems } from "@/api/item"

// ── 加载状态 ──────────────────────────────────────
const items = ref<ItemData[]>([])
const loading = ref(true)

onMounted(async () => {
  try {
    items.value = await listItems()
  } finally {
    loading.value = false
  }
})

// ── 衍生的展示字段 ────────────────────────────────
interface ItemDisplay extends ItemData {
  total_price: number
  total_weight: number
}

const displayItems = computed<ItemDisplay[]>(() =>
  items.value.map((item) => ({
    ...item,
    total_price: item.unit_price * item.quantity,
    total_weight: parseFloat((item.unit_weight * item.quantity).toFixed(2)),
  })),
)

// ── Columns ─────────────────────────────────────────
const columns: ColumnDef<ItemDisplay>[] = [
  { accessorKey: "name", header: "Name", enableSorting: true },
  { accessorKey: "category", header: "Category", enableSorting: true },
  {
    accessorKey: "unit_price",
    header: "Unit Price",
    enableSorting: true,
    cell: ({ row }) => `${row.getValue("unit_price")} g`,
    meta: { className: "text-right tabular-nums" },
  },
  {
    accessorKey: "quantity",
    header: "Qty",
    enableSorting: true,
    meta: { className: "text-right tabular-nums" },
  },
  {
    accessorKey: "total_price",
    header: "Total Price",
    enableSorting: true,
    cell: ({ row }) => `${row.getValue("total_price")} g`,
    meta: { className: "text-right tabular-nums font-semibold" },
  },
  {
    accessorKey: "unit_weight",
    header: "Unit Wt",
    enableSorting: true,
    meta: { className: "text-right tabular-nums text-muted-foreground" },
  },
  {
    accessorKey: "total_weight",
    header: "Total Wt",
    enableSorting: true,
    meta: { className: "text-right tabular-nums text-muted-foreground" },
  },
]

// ── Table state ─────────────────────────────────────
const sorting = ref<SortingState>([])

const table = useVueTable({
  get data() { return displayItems.value },
  get columns() { return columns },
  getCoreRowModel: getCoreRowModel(),
  getSortedRowModel: getSortedRowModel(),
  onSortingChange: (updaterOrValue) => valueUpdater(updaterOrValue, sorting),
  state: { get sorting() { return sorting.value } },
})
</script>

<template>
  <div v-if="loading" class="flex items-center justify-center py-16">
    <IconLoader2 class="size-8 animate-spin text-muted-foreground/50" />
  </div>

  <div v-else class="rounded-md border">
    <Table>
      <TableHeader>
        <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
          <TableHead v-for="header in headerGroup.headers" :key="header.id">
            <button
              v-if="header.column.getCanSort()"
              class="flex items-center gap-1 select-none"
              @click="header.column.toggleSorting(header.column.getIsSorted() === 'asc')"
            >
              <FlexRender
                :render="header.column.columnDef.header"
                :props="header.getContext()"
              />
              <span class="text-muted-foreground/50 ml-1">
                <IconArrowUp v-if="header.column.getIsSorted() === 'asc'" class="size-3.5" />
                <IconArrowDown v-else-if="header.column.getIsSorted() === 'desc'" class="size-3.5" />
                <IconArrowsUpDown v-else class="size-3.5" />
              </span>
            </button>
            <FlexRender v-else :render="header.column.columnDef.header" :props="header.getContext()" />
          </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <template v-if="table.getRowModel().rows.length">
          <TableRow
            v-for="(row, index) in table.getRowModel().rows"
            :key="row.id"
            :class="index % 2 === 0 ? 'bg-muted/30' : ''"
          >
            <TableCell
              v-for="cell in row.getVisibleCells()"
              :key="cell.id"
              :class="(cell.column.columnDef.meta as Record<string, string>)?.className"
            >
              <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()" />
            </TableCell>
          </TableRow>
        </template>
        <TableRow v-else>
          <TableCell :colspan="columns.length" class="h-24 text-center">
            No items available.
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
