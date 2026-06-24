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
} from "@tabler/icons-vue"
import { computed, ref } from "vue"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { valueUpdater } from "@/components/ui/table/utils"

// ── Types ──────────────────────────────────────────
interface InventoryItem {
  name: string
  category: string
  unitPrice: number
  quantity: number
  unitWeight: number
}

interface InventoryItemDisplay {
  name: string
  category: string
  unitPrice: number
  quantity: number
  unitWeight: number
  totalPrice: number
  totalWeight: number
}

// ── Mock data ───────────────────────────────────────
const items: InventoryItem[] = [
  { name: "Iron Longsword", category: "Weapons", unitPrice: 120, quantity: 2, unitWeight: 3.5 },
  { name: "Elven Bow", category: "Weapons", unitPrice: 250, quantity: 1, unitWeight: 2.0 },
  { name: "Leather Armor", category: "Armor", unitPrice: 80, quantity: 1, unitWeight: 5.0 },
  { name: "Steel Shield", category: "Armor", unitPrice: 180, quantity: 1, unitWeight: 4.5 },
  { name: "Health Potion", category: "Potions", unitPrice: 25, quantity: 5, unitWeight: 0.3 },
  { name: "Antidote", category: "Potions", unitPrice: 40, quantity: 4, unitWeight: 0.2 },
  { name: "Moonstone", category: "Materials", unitPrice: 150, quantity: 3, unitWeight: 0.8 },
  { name: "Mana Crystal", category: "Materials", unitPrice: 300, quantity: 2, unitWeight: 0.5 },
  { name: "Fireball Scroll", category: "Scrolls", unitPrice: 200, quantity: 1, unitWeight: 0.1 },
  { name: "Gold Coins (bag)", category: "Treasure", unitPrice: 500, quantity: 1, unitWeight: 1.2 },
]

// ── Computed ───────────────────────────────────────
const displayItems = computed<InventoryItemDisplay[]>(() =>
  items.map((item) => ({
    ...item,
    totalPrice: item.unitPrice * item.quantity,
    totalWeight: parseFloat((item.unitWeight * item.quantity).toFixed(2)),
  })),
)

// ── Columns ─────────────────────────────────────────
const columns: ColumnDef<InventoryItemDisplay>[] = [
  { accessorKey: "name", header: "Name", enableSorting: true },
  { accessorKey: "category", header: "Category", enableSorting: true },
  {
    accessorKey: "unitPrice",
    header: "Unit Price",
    enableSorting: true,
    cell: ({ row }) => `${row.getValue("unitPrice")} g`,
    meta: { className: "text-right tabular-nums" },
  },
  {
    accessorKey: "quantity",
    header: "Qty",
    enableSorting: true,
    meta: { className: "text-right tabular-nums" },
  },
  {
    accessorKey: "totalPrice",
    header: "Total Price",
    enableSorting: true,
    cell: ({ row }) => `${row.getValue("totalPrice")} g`,
    meta: { className: "text-right tabular-nums font-semibold" },
  },
  {
    accessorKey: "unitWeight",
    header: "Unit Wt",
    enableSorting: true,
    meta: { className: "text-right tabular-nums text-muted-foreground" },
  },
  {
    accessorKey: "totalWeight",
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
  <div class="rounded-md border">
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
            No results.
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>
