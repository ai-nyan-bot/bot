export * from './compare';
export * from './compose';
export * from './condition';

export enum RuleStatus {
    ACTIVE = 'ACTIVE',
    INACTIVE = 'INACTIVE',
    ACTIVE_EXHAUSTED = 'ACTIVE_EXHAUSTED',
    INACTIVE_EXHAUSTED = 'INACTIVE_EXHAUSTED'
}