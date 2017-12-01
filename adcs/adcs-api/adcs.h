/*
 * Copyright (C) 2017 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/**
 * @defgroup ADCS Kubos ADCS Interface
 * @addtogroup ADCS
 * @{
 */

#pragma once

#include <stdint.h>
#include "adcs-impl.h"

/* Define the global functions */
/**
 * Initialize the ADCS interface
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_init(void);
/**
 * Terminate the ADCS interface
 */
void k_adcs_terminate(void);
/**
 * Configure the ADCS
 * @note This function might not be implemented for all ADCSs. See specific ADCS API documentation for configuration availability and structure
 * @param [in] config Pointer to the ADCS configuration structure
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_configure(adcs_config * config, uint8_t count);
/**
 * Reset the ADCS
 * @note This function might not be implemented for all ADCSs
 * @param [in] type Type of reset to perform (hard, soft, etc)
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_reset(KADCSReset type);
/**
 * Read ADCS telemetry values
 * @note See specific ADCS API documentation for available telemetry types
 * @param [in] buffer Pointer to structure which data should be copied to
 * @param [in] type Telemetry packet to read
 * @return KADCSStatus ADCS_OK if OK, error otherwise
 */
KADCSStatus k_adcs_get_telemetry(adcs_telem * buffer, ADCSTelemType type);

KADCSStatus k_adcs_set_mode(ADCSMode mode, adcs_mode_param * params);

KADCSStatus k_adcs_get_mode(ADCSMode * mode);

KADCSStatus k_adcs_run_test(ADCSTestType test, adcs_test_results * data);

KADCSStatus k_adcs_passthrough(uint8_t * tx, int tx_len, uint8_t * rx, int rx_len);

KADCSStatus k_adcs_get_power_status(adcs_power_status * data);

KADCSStatus k_adcs_noop();

KADCSStatus k_adcs_get_orientation(adcs_orient * data);

KADCSStatus k_adcs_get_spin(adcs_spin * data);

/* @} */
