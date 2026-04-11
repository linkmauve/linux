/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/uapi/drm/flipper_drm.h
 *
 * Copyright (C) 2026 Link Mauve
 * Author: Link Mauve <linkmauve@linkmauve.fr>
 */

#ifndef __FLIPPER_DRM_H__
#define __FLIPPER_DRM_H__

#include "drm.h"

#if defined(__cplusplus)
extern "C" {
#endif

enum mem_type {
	BP = 0,
	CP,
	XF,
	XF_REGS,
	TEX_MEM,
};

struct drm_flipper_write_mem {
	enum mem_type mem_type;
	// This pointer is to little-endian data.
	const void *pointer;
	u32 size;
	u32 reserved;
};

enum command {
	FIFO = 0,
	UPDATE_VERTEX,
};

struct drm_flipper_cmd {
	enum command command;
	// This pointer is to big-endian data.
	const void *pointer;
	u32 size;
	u32 reserved;
};

struct drm_flipper_run_cmds {
	const struct drm_flipper_cmd cmds[4];
	u32 cmds_len;
};

enum {
	DRM_FLIPPER_WRITE_MEM		= 0x00,
	DRM_FLIPPER_RUN_CMDS		= 0x01,
	DRM_FLIPPER_NUM_IOCTLS		= 0x02,
};

/* Note: this is an enum so that it can be resolved by Rust bindgen. */
enum {
	DRM_IOCTL_FLIPPER_WRITE_MEM	= DRM_IOW (DRM_COMMAND_BASE + DRM_FLIPPER_WRITE_MEM, struct drm_flipper_write_mem),
	DRM_IOCTL_FLIPPER_RUN_CMDS	= DRM_IOW (DRM_COMMAND_BASE + DRM_FLIPPER_RUN_CMDS, struct drm_flipper_run_cmds),
};

#if defined(__cplusplus)
}
#endif

#endif /* __FLIPPER_DRM_H__ */
